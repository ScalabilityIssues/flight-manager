use std::error::Error;

use crate::{errors::ApplicationError, proto::flightmngr::Flight};
use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicPublishArguments, Channel, ExchangeDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    BasicProperties, FieldTable,
};
use prost::Message;

pub struct Rabbit {
    _connection: Connection,
    channel: Channel,
    exchange_name: String,
}

impl Rabbit {
    pub async fn new(
        rabbitmq_url: &str,
        rabbitmq_port: u16,
        rabbitmq_user: &str,
        rabbitmq_password: &str,
        exchange_name: String,
        exchange_type: String,
    ) -> Result<Self, Box<dyn Error>> {
        // open a connection to RabbitMQ server
        let rabbitmq = Connection::open(&OpenConnectionArguments::new(
            rabbitmq_url,
            rabbitmq_port,
            rabbitmq_user,
            rabbitmq_password,
        ))
        .await?;

        // Register connection level callbacks.
        // TODO: In production, user should create its own type and implement trait `ConnectionCallback`.
        rabbitmq
            .register_callback(DefaultConnectionCallback)
            .await?;

        // open a channel on the connection
        let rabbitmq_channel = rabbitmq.open_channel(None).await?;
        rabbitmq_channel
            .register_callback(DefaultChannelCallback)
            .await?;

        // declare the exchange in which to publish new or modified tickets
        rabbitmq_channel
            .exchange_declare(ExchangeDeclareArguments {
                exchange: exchange_name.clone(),
                exchange_type: String::from(exchange_type),
                passive: false, // if does not exist, then is created. If set to true, an error is raised if exchange does not exist
                durable: true,  // survive broker restart
                auto_delete: false, // survive even if no queue is bound
                internal: false,
                no_wait: false,
                arguments: FieldTable::default(),
            })
            .await?;

        Ok(Rabbit {
            _connection: rabbitmq,
            channel: rabbitmq_channel,
            exchange_name,
        })
    }

    pub async fn notify_flight_update(&self, message: &Flight) -> Result<(), ApplicationError> {
        let message = message.encode_to_vec();

        // create arguments for basic_publish
        let args = BasicPublishArguments::new(&self.exchange_name, "");
        let properties = BasicProperties::default().finish();
        self.channel
            .basic_publish(properties, message, args)
            .await?;

        Ok(())
    }
}
