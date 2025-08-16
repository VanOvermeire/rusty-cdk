// #[derive(Serialize)]
// pub enum Effect {
//     Allow,
//     Deny
// }
// 
// #[derive(Serialize)]
// pub enum Service {
//     Lambda
// }
// 
// impl From<Service> for String {
//     fn from(value: Service) -> Self {
//         match value {
//             Service::Lambda => "lambda.amazonaws.com".to_string()
//         }
//     }
// }