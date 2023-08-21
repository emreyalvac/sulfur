# Sulfur
![alt text](sulfur.jpeg)

#### WIP

https://www.youtube.com/watch?v=PAAvNmoqDq0 <br/>

## "Shaping, Processing, and Transforming Data with the Power of Sulfur"

Welcome to the Sulfur project, where we harness the elemental power of data transformation. Just like sulfur can reshape
its form, our platform reshapes, processes, and transforms data, turning it into valuable insights.

Join us on this journey of alchemy where data turns into gold through customization and innovation. Unleash the
potential of Sulfur and turn raw data into refined intelligence.

## Usage

1. Clone the repository:

    ```sh
    git clone https://github.com/emreyalvac/sulfur.git
    ```

2. Navigate to the project directory:

    ```sh
    cd sulfur-sink
    ```

3. Install dependencies:

    ```sh
    cargo build
    ```

4. Configure `config.yml` for your pipelines.

5. To run pipelines, use the terminal:

    ```sh
    cargo run -- --config config.yml
    ```

## Configuration (`config.yml`)

Here's an example of how to configure your pipelines using the `config.yml` file:

```yaml
sulfur:
  - name: "Mongo to Redis"
    cron: "0 0 * * *"
    source:
      type: "Mongo"
      host: "example.com"
      port: "5432"
      user: "user"
      password: "password"
      database: "db_name"
      collection: "collection_name"
    destination:
      type: "Redis"
      host: "redis.example.com"
      port: "6379"
      password: "redis_password"
      key: "data_key"      
```

### Configuration Details

- Each pipeline is defined under the `sulfur` key.
- `name`: A descriptive name for your pipeline.
- `cron` (optional): The cron expression to schedule pipeline runs (e.g., "0 0 * * *" for daily runs).
- `source`: Specifies the data source configuration.
- `destination`: Specifies the data destination configuration.

For the `source` and `destination` configurations, you can specify various parameters based on the type of engine you're
using (e.g., "Database," "Redis," "BigQuery," etc.).

Remember to customize the configuration according to your project's specific requirements.

## Transform Data, Shape Intelligence

Sulfur is more than a project; it's a catalyst for data alchemy. Whether you're merging, filtering, or aggregating,
Sulfur empowers you to sculpt raw data into refined insights, making your data truly valuable.

## Supported Engines

Sulfur currently supports the following data storage engines:

### Elasticsearch

- **Type:** Database
- **Description:** Use Elasticsearch as a data source or destination.
- **Parameters:** `host`, `port`, `user`, `password`, `index`

### MongoDB

- **Type:** Database
- **Description:** Use MongoDB as a data source or destination.
- **Parameters:** `host`, `port`, `user`, `password`, `database`, `collection`

### Redis

- **Type:** In-Memory Data Store
- **Description:** Use Redis as a data source or destination.
- **Parameters:** `host`, `port`, `password`, `key`

### BigQuery

- **Type:** Data Warehouse
- **Description:** Use Google BigQuery as a data destination.
- **Parameters:** `project_id`, `dataset_id`, `table_id`, `credentials`

We're committed to expanding the list of supported engines to give you even more flexibility. Adding new platforms is a
straightforward process, allowing you to tailor Sulfur to your evolving data needs.

## Upcoming Storage Possibilities

At Sulfur, we're committed to expanding the range of supported storage engines to cater to your evolving needs. Here's a
sneak peek at some potential storage engines that might be added in the future:

1. Amazon S3
2. Microsoft Azure Blob Storage
3. PostgreSQL
4. MySQL
5. SQLite
6. Cassandra
7. Apache Hadoop HDFS
8. Amazon Redshift
9. Snowflake
10. Apache Kafka
11. Oracle Database
12. IBM Db2
13. Microsoft SQL Server
14. Apache Hive
15. MongoDB Atlas
16. Elasticsearch Service
17. Redis Cloud
18. Memcached
19. InfluxDB
20. Kafka
21. RabbitMQ

Stay tuned as we continue to explore and add more storage engine options to the Sulfur platform. We're excited to
provide you with a broader range of choices for your data storage needs!

## Advanced Data Transformation

At Sulfur, we're dedicated to evolving our platform to meet your needs. We're excited to introduce a feature:

### How It Works

The Advanced Data Transformation feature will provide a powerful toolkit for crafting precise data transformations. From
mathematical operations to conditional logic, this feature grants you unparalleled control over your data.

Stay tuned as we work diligently to unveil this enhancement. Your data transformation possibilities are about to expand
like never before!

## Advanced Data Transformation using Python

Sulfur enables you to harness the power of custom Python scripts for advanced data transformation during the pipeline process. By integrating Python scripts, you can perform complex data manipulations, calculations, and enrichments before the data is forwarded to its destination.

### Implementing Advanced Transformation

To demonstrate the power of custom Python scripts for data transformation, we've provided an example `advanced_transform` function that showcases a basic transformation:

```python
import json

def advanced_transform(*args):
   # Unpack the arguments tuple
   data_string = args[0]

   # Load the JSON data
   data = json.loads(data_string)

   # Perform your advanced transformation here
   transformed_data = {
      "name": "TRANSFORMED",
      "original_data": data
   }

   # Convert the transformed data back to a JSON string
   transformed_json = json.dumps(transformed_data)

   return transformed_json
```

```yaml
sulfur:
  - name: "Pipeline1"
    cron: "0 0 * * *"
    transform:
      file: './transform.py'
      fn: 'advanced_transform'
    source:
      type: "MongoDB"
      host: "mongodb.example.com"
      port: "27017"
      user: "user"
      password: "password"
      database: "db_name"
      collection: "collection_name"
    destination:
      type: "Redis"
      host: "redis.example.com"
      port: "6379"
      password: "redis_password"
      key: "data_key"

```

## Contributing

See [Contributing Guidelines](CONTRIBUTING.md) for details on how to contribute to this project.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
