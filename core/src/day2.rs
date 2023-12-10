use serde::{Serialize, Deserialize, Serializer, ser::SerializeStruct};

#[derive(Clone, Serialize, Deserialize)]
pub struct Reindeer {
    name: String,
    strength: u32,
    pub speed: Option<f32>,
    pub height: Option<u32>,
    antler_width: Option<u32>,
    pub snow_magic_power: Option<u32>,
    favorite_food: Option<String>,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    pub  candies_eaten_yesterday: Option<u32>,
}

pub struct ReindeerContestSummary {
    pub fastest: Option<Reindeer>,
    pub tallest: Option<Reindeer>,
    pub magician: Option<Reindeer>,
    pub consumer: Option<Reindeer>,
}

impl Serialize for ReindeerContestSummary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ReindeerContestSummary", 4)?;

        if let Some(fastest) = &self.fastest {
            state.serialize_field("fastest", &format!("Speeding past the finish line with a strength of {} is {}", fastest.strength, fastest.name))?;
        }

        if let Some(tallest) = &self.tallest {
            state.serialize_field("tallest", &format!("{} is standing tall with his {} cm wide antlers", tallest.name, tallest.antler_width.unwrap()))?;
        }

        if let Some(magician) = &self.magician {
            state.serialize_field("magician", &format!("{} could blast you away with a snow magic power of {}", magician.name, magician.snow_magic_power.unwrap()))?;
        }

        if let Some(consumer) = &self.consumer {
            state.serialize_field("consumer", &format!("{} ate lots of candies, but also some {}", consumer.name, consumer.favorite_food.as_ref().unwrap()))?;
        }

        state.end()
    }
}

pub trait IntoReindeerContestSummary {
    fn contest(self) -> ReindeerContestSummary;
}

impl<T> IntoReindeerContestSummary for T 
where
    T: IntoIterator<Item = Reindeer>,
    T::IntoIter: Clone,
{
    fn contest(self) -> ReindeerContestSummary {
        let reindeers = self.into_iter();
        ReindeerContestSummary{
            fastest: reindeers.clone().max_by(|a, b| a.speed.unwrap().total_cmp(&b.speed.unwrap())),
            tallest: reindeers.clone().max_by_key(|r| r.height),
            magician: reindeers.clone().max_by_key(|r| r.snow_magic_power),
            consumer: reindeers.max_by_key(|r| r.candies_eaten_yesterday),
        }
    }
}

pub fn strength_sum(reindeers: impl IntoIterator<Item = Reindeer>) -> u32 {
    reindeers.into_iter().map(|x| x.strength).sum::<u32>()
}