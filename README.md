## ReMusic 2.0

ReMusic 2.0 is the next evolution of the ReMusic project, designed to scan physical sheet music and convert it into an editable digital format. This version introduces significant improvements in both performance and user experience:

- **Rust Backend:** Harnessing the power of Rust for blazing-fast processing and enhanced scalability.
- **React Frontend:** A modern, interactive, and maintainable user interface for seamless navigation and real-time interaction.
- **Containerized Deployment:** Built to run in Docker containers, making deployment straightforward and environment-agnostic.

### Why ReMusic 2.0?

1. **Speed and Performance**
   Rust's low-level memory management and zero-cost abstractions provide exceptional speed and reliability. This allows for faster processing of scanned music sheets and smoother data handling.

2. **Interactive Frontend**
   The shift to React means a richer, more engaging user experience with responsive design and intuitive interactions. Users can view, edit, and play music sheets more fluidly than ever.

3. **Scalability and Maintainability**
   With its component-based structure, the frontend is easier to maintain and extend. Meanwhile, the Rust backend is ready for high concurrency, handling more users and larger data with ease.

4. **Effortless Deployment**
   Containerization via Docker ensures ReMusic 2.0 is easy to deploy across different environments, reducing setup complexities and improving consistency.

### Installation Instructions

After cloning the repository, follow these steps to set up ReMusic 2.0. The order of operations is important.

#### Podman Virtual Network

```bash
# Use this command to use the preconfigured ENV
podman network create mynetwork
```

#### Database

```bash
# Use this command to use the preconfigured ENV
podman run -d --name postgresdb --network mynetwork -e POSTGRES_DB=postgres -e POSTGRES_USER=user -e POSTGRES_PASSWORD=password -p 6432:5432 postgres:17.4-alpine
```

#### Local Dev Enviorment

##### Kafka Instance

```bash
# Use this command to use the preconfigured ENV
podman run -d -p 9092:9092 --name broker --network mynetwork -e KAFKA_NODE_ID=1 -e KAFKA_PROCESS_ROLES=broker,controller -e KAFKA_LISTENERS=PLAINTEXT://0.0.0.0:9092,CONTROLLER://:9093 -e KAFKA_ADVERTISED_LISTENERS=PLAINTEXT://localhost:9092 -e KAFKA_CONTROLLER_LISTENER_NAMES=CONTROLLER -e KAFKA_LISTENER_SECURITY_PROTOCOL_MAP=CONTROLLER:PLAINTEXT,PLAINTEXT:PLAINTEXT -e KAFKA_CONTROLLER_QUORUM_VOTERS=1@localhost:9093 -e KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR=1 -e KAFKA_TRANSACTION_STATE_LOG_REPLICATION_FACTOR=1 -e KAFKA_TRANSACTION_STATE_LOG_MIN_ISR=1 -e KAFKA_GROUP_INITIAL_REBALANCE_DELAY_MS=0 -e KAFKA_NUM_PARTITIONS=1 apache/kafka:latest
```

##### Backend

```bash
# To install, build and run the Rust API
cd ReMusic 2.0/Backend/remusic-back/
cargo build
cargo run
```

##### Frontend

```bash
# To install and run the React frontend
cd ReMusic 2.0/Frontend/remusic-front/
npm install
npm run dev
```

#### Prod Enviorment

##### Kafka Instance

```bash
# Use this command to use the preconfigured ENV
podman run -d -p 9092:9092 --name broker --network mynetwork -e KAFKA_NODE_ID=1 -e KAFKA_PROCESS_ROLES=broker,controller -e KAFKA_LISTENERS=PLAINTEXT://:9092,CONTROLLER://:9093 -e KAFKA_ADVERTISED_LISTENERS=PLAINTEXT://broker:9092 -e KAFKA_CONTROLLER_LISTENER_NAMES=CONTROLLER -e KAFKA_LISTENER_SECURITY_PROTOCOL_MAP=CONTROLLER:PLAINTEXT,PLAINTEXT:PLAINTEXT -e KAFKA_CONTROLLER_QUORUM_VOTERS=1@localhost:9093 -e KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR=1 -e KAFKA_TRANSACTION_STATE_LOG_REPLICATION_FACTOR=1 -e KAFKA_TRANSACTION_STATE_LOG_MIN_ISR=1 -e KAFKA_GROUP_INITIAL_REBALANCE_DELAY_MS=0 -e KAFKA_NUM_PARTITIONS=1 apache/kafka:latest
```

### Technologies Used

- **Rust** — For backend processing and API management.
- **React** — For an interactive and modern frontend.
- **Podman** — For isolated and consistent deployments.
- **Postgres** - For database management and data storage.

#### Needed Software

- **Rustup and Cargo** - For building and running the Rust API.
- **Podman** — For isolated and consistent deployments.
- **Npm and Node** - For building and running the React frontend.

### Contributing

Contributions are welcome! Feel free to submit issues and pull requests to improve the project.

### License

This project is licensed under the LGPL-3.0 license.
