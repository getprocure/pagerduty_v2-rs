use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use super::reference::Reference;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ContactMethodUnion {
    // All Reference's
    id: String,
    summary: String,
    #[serde(rename="type")]
    type_: String,
    #[serde(rename="self")]
    self_: String,
    html_url: Option<String>,

    // All Concrete type fields
    address: Option<String>,
    label: Option<String>,
    send_short_email: Option<bool>,
    send_html_email: Option<bool>,
    blacklisted: Option<bool>,
    country_code: Option<u32>,
    enabled: Option<bool>,
    created_at: Option<String>,
    device_type: Option<String>,
    sounds: Option<Vec<PushContactMethodSound>>,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PushContactMethodSound {
    /// The sound file name.
    pub file: String,

    /// The type of sound. Expected values include:
    /// `alert_high_urgency` and `alert_high_urgency`.
    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Debug, PartialEq)]
pub enum ContactMethod {
    ContactMethodReference {
        reference: Reference,
    },

    EmailContactMethod{
        reference: Reference,

        /// The `address` to deliver to: email, phone number, etc.,
        ///  depending on the type.
        address: String,

        /// The label (e.g., "Work", "Mobile", etc.).
        label: String,

        /// Send an abbreviated email message instead of the standard email
        /// output. Useful for email-to-SMS gateways and email based pagers.
        send_short_email: bool,

        /// Send HTML e-mails.
        send_html_email: bool,
    },

    PhoneContactMethod{
        reference: Reference,

        /// The `address` to deliver to: email, phone number, etc.,
        ///  depending on the type.
        address: String,

        /// The label (e.g., "Work", "Mobile", etc.).
        label: String,

        /// If true, this phone has been blacklisted by
        /// PagerDuty and no messages will be sent to it.
        blacklisted: bool,

        /// The 1-to-3 digit country calling code.
        country_code: u32,
    },

    SmsContactMethod{
        reference: Reference,

        /// The `address` to deliver to: email, phone number, etc.,
        ///  depending on the type.
        address: String,

        /// The label (e.g., "Work", "Mobile", etc.).
        label: String,

        /// If true, this phone has been blacklisted by
        /// PagerDuty and no messages will be sent to it.
        blacklisted: bool,

        /// The 1-to-3 digit country calling code.
        country_code: u32,

        /// If true, this phone is capable of receiving SMS messages.
        enabled: bool,
    },

    PushNotificationContactMethod{
        reference: Reference,

        /// The `address` to deliver to: email, phone number, etc.,
        ///  depending on the type.
        address: String,

        /// The label (e.g., "Work", "Mobile", etc.).
        label: String,

        /// If true, this phone has been blacklisted by PagerDuty and no messages will be sent to it.",
        blacklisted: bool,

        // TODO(gary): Use date-time field?
        /// Time at which the contact method was created.
        created_at: String,

        /// The type of device. Expected values include:
        /// `ios` and `android`.
        device_type: String,

        sounds: Vec<PushContactMethodSound>,
    },

}


impl Serialize for ContactMethod {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_map(None)?;

        match *self {
            ContactMethod::ContactMethodReference{
                ref reference
            } => {
                reference.serialize_key_vals(serializer, &mut state)?;
            },
            ContactMethod::EmailContactMethod{
                ref reference, ref address, ref label,
                ref send_short_email, ref send_html_email,
            } => {
                reference.serialize_key_vals(serializer, &mut state)?;

                serializer.serialize_map_key(&mut state, "address")?;
                serializer.serialize_map_value(&mut state, address)?;

                serializer.serialize_map_key(&mut state, "label")?;
                serializer.serialize_map_value(&mut state, label)?;

                serializer.serialize_map_key(&mut state, "send_short_email")?;
                serializer.serialize_map_value(&mut state, send_short_email)?;

                serializer.serialize_map_key(&mut state, "send_html_email")?;
                serializer.serialize_map_value(&mut state, send_html_email)?;
            },
            ContactMethod::PhoneContactMethod{
                ref reference, ref address, ref label,
                ref blacklisted, ref country_code,
            } => {
                reference.serialize_key_vals(serializer, &mut state)?;

                serializer.serialize_map_key(&mut state, "address")?;
                serializer.serialize_map_value(&mut state, address)?;

                serializer.serialize_map_key(&mut state, "label")?;
                serializer.serialize_map_value(&mut state, label)?;

                serializer.serialize_map_key(&mut state, "country_code")?;
                serializer.serialize_map_value(&mut state, country_code)?;

                serializer.serialize_map_key(&mut state, "blacklisted")?;
                serializer.serialize_map_value(&mut state, blacklisted)?;

            },
            ContactMethod::SmsContactMethod{
                ref reference, ref address, ref label,
                ref blacklisted, ref country_code, ref enabled,
            } => {
                reference.serialize_key_vals(serializer, &mut state)?;

                serializer.serialize_map_key(&mut state, "address")?;
                serializer.serialize_map_value(&mut state, address)?;

                serializer.serialize_map_key(&mut state, "label")?;
                serializer.serialize_map_value(&mut state, label)?;

                serializer.serialize_map_key(&mut state, "country_code")?;
                serializer.serialize_map_value(&mut state, country_code)?;

                serializer.serialize_map_key(&mut state, "blacklisted")?;
                serializer.serialize_map_value(&mut state, blacklisted)?;

                serializer.serialize_map_key(&mut state, "enabled")?;
                serializer.serialize_map_value(&mut state, enabled)?;
            },
            ContactMethod::PushNotificationContactMethod{
                ref reference, ref address, ref label,
                ref blacklisted, ref created_at, ref device_type,
                ref sounds,
            } => {
                reference.serialize_key_vals(serializer, &mut state)?;

                serializer.serialize_map_key(&mut state, "address")?;
                serializer.serialize_map_value(&mut state, address)?;

                serializer.serialize_map_key(&mut state, "label")?;
                serializer.serialize_map_value(&mut state, label)?;

                serializer.serialize_map_key(&mut state, "device_type")?;
                serializer.serialize_map_value(&mut state, device_type)?;

                serializer.serialize_map_key(&mut state, "sounds")?;
                serializer.serialize_map_value(&mut state, sounds)?;

                serializer.serialize_map_key(&mut state, "blacklisted")?;
                serializer.serialize_map_value(&mut state, blacklisted)?;

                serializer.serialize_map_key(&mut state, "created_at")?;
                serializer.serialize_map_value(&mut state, created_at)?;
            },
        }

        serializer.serialize_map_end(state)
    }
}

impl Deserialize for ContactMethod {
    fn deserialize<D>(deserializer: &mut D) -> Result<ContactMethod, D::Error>
        where D: Deserializer
    {
        let union = ContactMethodUnion::deserialize(deserializer)?;

        let reference = Reference {
            id: union.id,
            summary: union.summary,
            type_: union.type_,
            self_: union.self_,
            html_url: union.html_url,
        };

        match reference.type_.as_ref() {
            "contact_method_reference" |
            "email_contact_method_reference" |
            "phone_contact_method_reference" |
            "sms_contact_method_reference" |
            "push_notification_contact_method_reference"
            => {
                Ok(ContactMethod::ContactMethodReference {
                    reference: reference,
                })
            },
            "email_contact_method" => {
                Ok(ContactMethod::EmailContactMethod {
                    reference: reference,
                    address: union.address.expect("address"),
                    label: union.label.expect("label"),
                    send_short_email: union.send_short_email.expect("send_short_email"),
                    send_html_email: union.send_html_email.expect("send_html_email"),
                })
            },
            "phone_contact_method" => {
                Ok(ContactMethod::PhoneContactMethod {
                    reference: reference,
                    address: union.address.expect("address"),
                    label: union.label.expect("label"),
                    blacklisted: union.blacklisted.expect("blacklisted"),
                    country_code: union.country_code.expect("country_code"),
                })
            },
            "sms_contact_method" => {
                Ok(ContactMethod::SmsContactMethod {
                    reference: reference,
                    address: union.address.expect("address"),
                    label: union.label.expect("label"),
                    blacklisted: union.blacklisted.expect("blacklisted"),
                    country_code: union.country_code.expect("country_code"),
                    enabled: union.enabled.expect("enabled"),
                })
            },
            "push_notification_contact_method" => {
                Ok(ContactMethod::PushNotificationContactMethod {
                    reference: reference,
                    address: union.address.expect("address"),
                    label: union.label.expect("label"),
                    blacklisted: union.blacklisted.expect("blacklisted"),
                    created_at: union.created_at.expect("created_at"),
                    device_type: union.device_type.expect("device_type"),
                    sounds: union.sounds.expect("sounds"),
                })
            },
            _ => panic!("fuuuuuuu"),
        }
    }
}


pub type ContactMethods = Vec<ContactMethod>;


#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;
    use std::fs::File;
    use std::io::Read;
    use super::super::reference::Reference;

    #[test]
    fn test_serde() {
        let mut file = File::open("testdata/types/contact_methods.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let contact_methods: ContactMethods = serde_json::from_str(&data).unwrap();

        // Verify deserialization.
        assert_eq!(
            contact_methods,
            vec![

                ContactMethod::ContactMethodReference {
                    reference: Reference {
                        id: "PPPIOPG".into(),
                        summary: "Default".into(),
                        type_: "email_contact_method_reference".into(),
                        self_: "https://api.pagerduty.com/users/PZ7JFQ7/contact_methods/PPPIOPG".into(),
                        html_url: None,
                    },
                },
                ContactMethod::EmailContactMethod {
                    reference: Reference {
                        id: "P33R0ZA".into(),
                        summary: "Work".into(),
                        type_: "email_contact_method".into(),
                        self_: "https://api.pagerduty.com/users/PZ7JFQ7/contact_methods/P33R0ZA".into(),
                        html_url: None,
                    },
                    address: "alejandro@example.com".into(),
                    label: "Work".into(),
                    send_short_email: false,
                    send_html_email: false,
                },
                ContactMethod::SmsContactMethod {
                    reference: Reference {
                        id: "PEC83HY".into(),
                        summary: "Mobile".into(),
                        type_: "sms_contact_method".into(),
                        self_: "https://api.pagerduty.com/users/PGJ36Z3/contact_methods/PEC83HY".into(),
                        html_url: None,
                    },
                    address: "4155809923".into(),
                    label: "Mobile".into(),
                    blacklisted: false,
                    country_code: 1,
                    enabled: true,
                },
                ContactMethod::PhoneContactMethod {
                    reference: Reference {
                        id: "PBUSVMD".into(),
                        summary: "Mobile".into(),
                        type_: "phone_contact_method".into(),
                        self_: "https://api.pagerduty.com/users/P1RQ0Z6/contact_methods/PBUSVMD".into(),
                        html_url: None,
                    },
                    address: "7076949626".into(),
                    label: "Mobile".into(),
                    blacklisted: false,
                    country_code: 1,
                },
                ContactMethod::PushNotificationContactMethod {
                    reference: Reference {
                        id: "P4G3JKD".into(),
                        summary: "Alex\'s iPhone".into(),
                        type_: "push_notification_contact_method".into(),
                        self_: "https://api.pagerduty.com/users/P1RQ0Z6/contact_methods/P4G3JKD".into(),
                        html_url: None,
                    },
                    address: "fcbaba06abe7533794b0dd7c3f4427b574772c01445e06bb5a006c33f14d95d0".into(),
                    label: "Alex\'s iPhone".into(),
                    blacklisted: false,
                    created_at: "2016-07-11T11:36:41-07:00".into(),
                    device_type: "ios".into(),
                    sounds: vec![
                        PushContactMethodSound {
                            file: "default".into(),
                            type_: "alert_high_urgency".into(),
                        }
                    ],
                }
            ]
        );

        // Verify that serialization round-trips.
        let expected: serde_json::Value = serde_json::from_str(&data).unwrap();
        let serialized: serde_json::Value = serde_json::from_str(
            serde_json::to_string(&contact_methods).unwrap().as_ref()
        ).unwrap();
        assert_eq!(serialized, expected)
    }
}
