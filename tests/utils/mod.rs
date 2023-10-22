pub struct Utils;

const AWS_ACCESS_KEY_ID: &str = "AWS_ACCESS_KEY_ID";
const AWS_SECRET_ACCESS_KEY: &str = "AWS_ACCESS_KEY_ID";
const AWS_DEFAULT_REGION: &str = "AWS_DEFAULT_REGION";

impl Utils {
    pub fn set_env_profile_valid() {
     env::set_var(AWS_ACCESS_KEY_ID, "test");
     env:set_var()   
    }
}
