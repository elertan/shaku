//! Modules and services can be generic. Based off of issue #2:
//! https://github.com/Mcat12/shaku/issues/2
#[macro_use]
extern crate async_trait;

use shaku::{module, Component, Interface, HasComponent};
use sqlx::{Connection, PgConnection, PgPool, Postgres, Transaction};
use std::env;
use std::sync::{Arc, Mutex};
use sqlx::pool::PoolConnection;

#[async_trait]
trait LogService: Interface {
    async fn log(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Component)]
#[shaku(interface = LogService)]
struct LogServiceImpl<C> where C: Connection<Database = Postgres> + Default + Interface {
    #[shaku(default = unreachable!())]
    conn: Arc<Mutex<C>>,
}

#[async_trait]
impl<C> LogService for LogServiceImpl<C> where C: Connection<Database = Postgres> + Default + Interface {
    async fn log(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.conn.get_mut()?;
        sqlx::query("INSERT INTO logs (message) VALUES ($1);")
            .bind(message)
            .execute(conn)
            .await?;
        Ok(())
    }
}

module! {
    MyModule<C: Connection<Database = Postgres> + Default + Interface> {
        components = [LogServiceImpl<C>],
        providers = []
    }
}

type ConnectionType = PoolConnection<PgConnection>;
type TransactionType = Transaction<ConnectionType>;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let pool = PgPool::builder()
        .build(&env::var("DATABASE_URL")?).await?;
    
    // Simple connection
    {
        let conn = pool.acquire().await?;
        let conn = Arc::new(Mutex::new(conn));

        // Create a module from a concrete
        let mut module = MyModule::<ConnectionType>::builder()
            .with_component_parameters::<LogServiceImpl<ConnectionType>>(LogServiceImplParameters {
                conn: conn.clone()
            })
            .build();

        let log_service: &mut dyn LogService = module.resolve_mut();
        log_service.log("Hello, world!").await?;
    }

    // Transactional operation
    {
        let tx = pool.begin().await?;
        let tx = Arc::new(Mutex::new(tx));

        let mut module = MyModule::<TransactionType>::builder()
            .with_component_parameters::<LogServiceImpl<TransactionType>>(LogServiceImplParameters {
                conn: tx.clone()
            })
            .build();

        let log_service: &mut dyn LogService = module.resolve_mut();
        log_service.log("Hello, world with transaction!").await?;

        tx.commit().await?;
    }

    Ok(())
}
