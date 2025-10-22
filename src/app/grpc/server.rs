use publish::publish_server::Publish;

pub mod publish {
    tonic::include_proto!("publish");
}

#[derive(Debug)]
pub struct PublishService {}


impl Publish for PublishService {

}