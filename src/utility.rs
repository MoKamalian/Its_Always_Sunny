/**
@author: amir kamalian
@date: 28 sep 2023
 */

mod Utility {

    // UI ACII art style function calls
    pub trait WeatherASCIIArt {
        fn cloudy(&self);
        fn raining(&self);
        fn drizzle(&self);
        fn sunny(&self);
        fn partially_sunny(&self);
        fn snowing(&self);
        fn snow_storm(&self);
    }

    struct City {
        lat: i64,
        lon: i64,
        weather: EnvData
    }

    struct EnvData {
        temp: i64,
        feels_like: i64,
        humidity: i64,
        pressure: i64,
        description: String
    }

    impl WeatherASCIIArt for City {
        fn cloudy(&self) {}
        fn raining(&self) {}
        fn drizzle(&self) {}
        fn sunny(&self) {}
        fn partially_sunny(&self) {}
        fn snowing(&self) {}
        fn snow_storm(&self) {}
    }

    /** API functions */

    // api call function

    // parser




}

