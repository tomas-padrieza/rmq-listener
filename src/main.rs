use futures_lite::stream::StreamExt;
use lapin::{Connection, ConnectionProperties, options::*, types::FieldTable};
use std::env;

fn load_env_var(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Missing environment variable: {}", key))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Build connection string from environment variables
    let host = load_env_var("RABBITMQ_HOST");
    let port = load_env_var("RABBITMQ_PORT");
    let username = load_env_var("RABBITMQ_USERNAME");
    let password = load_env_var("RABBITMQ_PASSWORD");
    let vhost = load_env_var("RABBITMQ_VHOST");

    let uri = format!(
        "amqp://{}:{}@{}:{}/{}",
        username, password, host, port, vhost
    );

    println!("Connecting to RabbitMQ...");

    // Connect to RabbitMQ
    let conn = Connection::connect(&uri, ConnectionProperties::default()).await?;

    println!("Connected to RabbitMQ");

    // Create a channel
    let channel = conn.create_channel().await?;

    // Get exchange and queue names from environment
    let exchange_name = load_env_var("RABBITMQ_EXCHANGE");
    let queue_names = load_env_var("RABBITMQ_QUEUE")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();

    // Declare an exchange
    channel
        .exchange_declare(
            &exchange_name,
            lapin::ExchangeKind::Topic,
            ExchangeDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let mut consumers = Vec::new();

    // Declare and bind each queue
    for queue_name in queue_names {
        let routing_key = format!("{}.#", queue_name);

        // Declare a queue
        let queue = channel
            .queue_declare(
                &queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        println!("Declared queue {}", queue.name());

        // Bind the queue to the exchange
        channel
            .queue_bind(
                &queue_name,
                &exchange_name,
                &routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        println!(
            "Queue {} bound to exchange with routing key: {}",
            queue_name, routing_key
        );

        // Create consumer for this queue
        let consumer = channel
            .basic_consume(
                &queue_name,
                &format!("consumer_{}", queue_name),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        consumers.push((queue_name, consumer));
    }

    println!("Starting to consume messages from all queues...");
    println!("Press Ctrl+C to exit");

    // Create a vector to hold all the consumer tasks
    let mut consumer_tasks = Vec::new();

    // Process messages from all consumers
    for (queue_name, mut consumer) in consumers {
        let task = tokio::spawn(async move {
            println!("Started consuming from queue: {}", queue_name);
            while let Some(delivery) = consumer.next().await {
                if let Ok(delivery) = delivery {
                    if let Ok(data) = std::str::from_utf8(&delivery.data) {
                        println!("Received message from {}: {}", queue_name, data);
                    }
                    delivery
                        .ack(BasicAckOptions::default())
                        .await
                        .expect("Failed to ack message");
                }
            }
        });
        consumer_tasks.push(task);
    }

    // Wait for all consumer tasks
    for task in consumer_tasks {
        if let Err(e) = task.await {
            eprintln!("Error in consumer task: {:?}", e);
        }
    }

    Ok(())
}
