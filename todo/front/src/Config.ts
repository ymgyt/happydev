class ConfigProvider {
  readonly todoApiUrl: string | undefined
  readonly version: string | undefined

  private constructor() {
    this.todoApiUrl = process.env.REACT_APP_TODO_API_URL;
    this.version = process.env.REACT_APP_VERSION || "v0.0.0";
  }

  static getInstance(): ConfigProvider {
    return new ConfigProvider()
  }
}

const Config = ConfigProvider.getInstance();
export default Config
