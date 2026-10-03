pub trait TextPlugin {
    fn process(&self, input: &str) -> String;
}

pub struct TrimPlugin;

impl TextPlugin for TrimPlugin {
    fn process(&self, input: &str) -> String {
        input.trim().to_string()
    }
}

pub struct PrefixPlugin {
    pub prefix: String,
}

impl TextPlugin for PrefixPlugin {
    fn process(&self, input: &str) -> String {
        format!("{}{}", self.prefix, input)
    }
}

// fn run_pipeline(text: &str, plugins: &[Box<dyn TextPlugin>]) -> String {
//     let mut current = text.to_string();
//     for plugin in plugins {
//         current = plugin.process(&current);
//     }
//     current
// }

fn run_pipeline(text: &str, plugins: &[Box<dyn TextPlugin>]) -> String {
    plugins.iter().fold(text.to_string(), |acc, plugin| {
        plugin.process(&acc)
    })
}

fn main() {
    // A Vec storing different concrete types via Box<dyn TextPlugin>
    let pipeline: Vec<Box<dyn TextPlugin>> = vec![
        Box::new(TrimPlugin),
        Box::new(PrefixPlugin {
            prefix: "[LOG]: ".to_string(),
        }),
    ];

    let result = run_pipeline("   System initialized   ", &pipeline);

    assert_eq!(result, "[LOG]: System initialized");
    println!("Trait Object Box Exercise Passed!");
}