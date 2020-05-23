class ConfigProvider {
  readonly todoApiUrl: string | undefined

  private constructor() {
    this.todoApiUrl = process.env.REACT_APP_TODO_API_URL;
  }

  static getInstance(): ConfigProvider {
    return new ConfigProvider()
  }
}

const Config = ConfigProvider.getInstance();
export default Config
