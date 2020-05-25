import Config from "../Config";

class TodoApiClient {
  constructor(private readonly url: string) {
    this.url = url;
  }

  async GET(path: string): Promise<any> {
    return fetch(this.endpoint(path)).then(res => res.json())
  }

  async POST(path: string, body: any): Promise<any> {
    return fetch(this.endpoint(path), {
      method: 'POST',
      body: JSON.stringify(body),
      headers: {
        'Content-Type': 'application/json',
      }
    })
  }

  private endpoint(path: string): string {
    return `${this.url}${path}`
  }
}

const TodoApi = new TodoApiClient(Config.todoApiUrl!)
export default TodoApi

