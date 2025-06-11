# RMQ Listener

A Rust-based RabbitMQ listener application that demonstrates how to work with RabbitMQ message queues using the `lapin` crate.

## Prerequisites

-   Docker and Docker Compose

## Setup and Running

### Using Docker Compose (Recommended)

1. Start the application and RabbitMQ using Docker Compose:

    ```bash
    docker compose up -d
    ```

    This will:

    - Start a RabbitMQ server with management UI
    - Build and start the RMQ listener application

2. Access RabbitMQ Management UI:
    - URL: http://localhost:15672
    - Username: guest
    - Password: guest

## Testing Message Publishing

You can test message publishing using several methods:

### 1. Using RabbitMQ Management UI

1. Open http://localhost:15672 in your browser
2. Log in with guest/guest
3. Go to "Queues" tab
4. Select your queue
5. Use the "Publish message" section to send test messages

### 2. Using the RabbitMQ CLI Tool (rabbitmqadmin)

1. Install rabbitmqadmin:

    ```bash
    curl -O http://localhost:15672/cli/rabbitmqadmin
    chmod +x rabbitmqadmin
    ```

2. Publish a test message:
    ```bash
    ./rabbitmqadmin publish exchange=amq.default routing_key=your_queue payload="Hello, World!"
    ```

## Environment Variables

The application uses the following environment variables:

-   `RABBITMQ_HOST`: RabbitMQ server hostname (default: "localhost")
-   `RABBITMQ_PORT`: RabbitMQ server port (default: 5672)
-   `RABBITMQ_USER`: RabbitMQ username (default: "guest")
-   `RABBITMQ_PASS`: RabbitMQ password (default: "guest")

## Docker Container Ports

-   RabbitMQ AMQP: 5672
-   RabbitMQ Management UI: 15672
