use crate::browser::Browser;

const DEFAULT_ARGS: [&'static str; 22] = [
  "--disable-background-networking",
  "--disable-background-timer-throttling",
  "--disable-backgrounding-occluded-windows",
  "--disable-breakpad",
  "--disable-client-side-phishing-detection",
  "--disable-default-apps",
  "--disable-dev-shm-usage",
  "--disable-extensions",
  // TODO: Support OOOPIF. @see https://github.com/GoogleChrome/puppeteer/issues/2548
  "--disable-features=site-per-process",
  "--disable-hang-monitor",
  "--disable-ipc-flooding-protection",
  "--disable-popup-blocking",
  "--disable-prompt-on-repost",
  "--disable-renderer-backgrounding",
  "--disable-sync",
  "--disable-translate",
  "--metrics-recording-only",
  "--no-first-run",
  "--safebrowsing-disable-auto-update",
  "--enable-automation",
  "--password-store=basic",
  "--use-mock-keychain",
];

#[derive(Debug)]
pub struct Viewport {
    pub width: i32,
    pub height: i32,
    pub deviceScaleFactor: i32,
    pub isMobile: bool,
    pub hasTouch: bool,
    pub isLandscape: bool,
}

#[derive(Debug)]
pub struct LaunchOptions {
    pub ignore_https_errors: bool,
    pub headless: bool,
    pub executable_path: Option<String>,
    pub slow_mo: i32,
    pub default_viewport: Option<Viewport>,
    pub args: Vec<String>,
    pub ignore_default_args: bool,
    pub timeout: i32,
    pub dumpio: bool,
    pub user_data_dir: Option<String>,
    pub env: Vec<(String, String)>,
    pub devtools: bool,
    pub pipe: bool,
}

impl LaunchOptions {
    pub fn new() -> LaunchOptions {
        LaunchOptions {
            ignore_https_errors: false,
            headless: true,
            executable_path: None,
            slow_mo: 0,
            default_viewport: None,
            args: Vec::new(),
            ignore_default_args: false,
            timeout: 30000,
            dumpio: false,
            user_data_dir: None,
            env: Vec::new(),
            devtools: false,
            pipe: false,
        }
    }
}

pub struct Launcher {
    project_root: Option<String>,
}

fn has(needle: &str, haystack: &Vec<String>) -> bool {
    for item in haystack {
        if item.starts_with(needle) {
            return true;
        }
    }
    false
}

impl Launcher {
    pub fn new() -> Launcher {
        Launcher {
            project_root: None,
        }
    }

    pub fn from_root(project_root: String) -> Launcher {
        Launcher {
            project_root: Some(project_root),
        }
    }

    pub async fn launch<'a>(&'a self, options: &'a LaunchOptions) -> Browser {

        // Ensure remote debugging argument is set
        let mut chrome_arguments = options.args.clone();
        if !has("--remote-debugging", &chrome_arguments) {
          let debug_argument = if options.pipe {
              String::from("--remote-debugging-pipe")
          } else {
              String::from("--remote-debugging-port=0")
          };
          chrome_arguments.push(debug_argument);
        }

        /*
        // Ensure user data dir argument is set
        let mut temporary_user_data_dir;
        if !has("--user-data-dir", chrome_arguments) {
          temporary_user_data_dir = await!(mkdtempAsync(CHROME_PROFILE_PATH));
          chrome_arguments.push(format!(
            "--user-data-dir={}",
            temporary_user_data_dir
          ));
        }
        */

        // Get executable
        let mut chrome_executable = options.executable_path.clone();
        if chrome_executable.is_none() {
          //chrome_executable = self.resolve_executable_path();
          //const {missingText, executablePath} = this._resolveExecutablePath();
          //if (missingText)
          //  throw new Error(missingText);
          //chromeExecutable = executablePath;
        }

        Browser {}
    }

    /*
    fn resolveExecutablePath(&self) -> String {
        let browserFetcher = BrowserFetcher::new(self.project_root);

        // puppeteer-core doesn't take into account PUPPETEER_* env variables.
        if (!this._isPuppeteerCore) {
          const executablePath = process.env['PUPPETEER_EXECUTABLE_PATH'];
          if (executablePath) {
            const missingText = !fs.existsSync(executablePath) ? 'Tried to use PUPPETEER_EXECUTABLE_PATH env variable to launch browser but did not find any executable at: ' + executablePath : null;
            return { executablePath, missingText };
          }
          const revision = process.env['PUPPETEER_CHROMIUM_REVISION'];
          if (revision) {
            const revisionInfo = browserFetcher.revisionInfo(revision);
            const missingText = !revisionInfo.local ? 'Tried to use PUPPETEER_CHROMIUM_REVISION env variable to launch browser but did not find executable at: ' + revisionInfo.executablePath : null;
            return {executablePath: revisionInfo.executablePath, missingText};
          }
        }
        const revisionInfo = browserFetcher.revisionInfo(this._preferredRevision);
        const missingText = !revisionInfo.local ? `Chromium revision is not downloaded. Run "npm install" or "yarn install"` : null;
        return {executablePath: revisionInfo.executablePath, missingText};
    }
    */
}