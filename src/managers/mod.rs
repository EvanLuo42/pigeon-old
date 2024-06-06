use std::any::{Any, type_name, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;
use tokio::sync::RwLock;
use crate::error::ServerError;
use crate::error::ServerError::ManagerNotExist;
use crate::managers::chat::ChatManager;

pub mod chat;

pub trait Manager: Any + Sync + Send {
    fn init() -> Self where Self: Sized;
}

lazy_static! {
    pub static ref MANAGER_CONTAINER: ManagerContainer = ManagerContainer::new();
}

pub async fn init() -> Result<(), ServerError> {
    Ok(MANAGER_CONTAINER.register(ChatManager::init()).await?)
}

pub async fn get_manager<M: Manager + 'static>() -> Result<Arc<M>, ServerError> {
    Ok(MANAGER_CONTAINER.get_manager::<M>().await?)
}

pub struct ManagerContainer {
    managers: RwLock<HashMap<TypeId, Arc<dyn Manager>>>,
}

impl ManagerContainer {
    fn new() -> Self {
        Self {
            managers: RwLock::new(HashMap::new()),
        }
    }

    async fn register<M: Manager + 'static>(&self, manager: M) -> Result<(), ServerError> {
        self.managers.write().await.insert(TypeId::of::<M>(), Arc::new(manager));
        Ok(())
    }

    async fn get_manager<M: Manager + 'static>(&self) -> Result<Arc<M>, ServerError> {
        if let Some(manager) = self.managers.read().await.get(&TypeId::of::<M>()) {
            return manager.clone().downcast_arc::<M>()
        }
        Err(ManagerNotExist(type_name::<M>().to_string()))
    }
}

trait Downcast {
    fn downcast_arc<M: Manager + 'static>(self: Arc<Self>) -> Result<Arc<M>, ServerError>;
}

impl Downcast for dyn Manager {
    fn downcast_arc<M: Manager + 'static>(self: Arc<Self>) -> Result<Arc<M>, ServerError> {
        if self.as_any().is::<M>() {
            let ptr = Arc::into_raw(self) as *const M;
            Ok(unsafe { Arc::from_raw(ptr) })
        } else {
            Err(ManagerNotExist(type_name::<M>().to_string()))
        }
    }
}

trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any + Send + Sync> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}