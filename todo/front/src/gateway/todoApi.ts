import Config from "../Config";

class TodoApiClient {
  constructor(private readonly url: string) {
    this.url = url;
  }

  async getTasks(param: any): Promise<any> {
    return this.GET('/tasks')
  }

  async createTask(param: any): Promise<any> {
    return this.POST('/tasks', param)
  }

  async deleteTask(taskId: string): Promise<any> {
    return this.DELETE(`/tasks/${taskId}`)
  }

  private async GET(path: string): Promise<any> {
    return fetch(this.endpoint(path)).then(res => res.json())
  }

  private async POST(path: string, body: any): Promise<any> {
    return fetch(this.endpoint(path), {
      method: 'POST',
      body: JSON.stringify(body),
      headers: {
        'Content-Type': 'application/json',
      }
    }).then(res => res.json())
  }

  private async DELETE(path: string): Promise<any> {
    return fetch(this.endpoint(path), {
        method: 'DELETE'
      }
    )
  }

  private endpoint(path: string): string {
    return `${this.url}${path}`
  }
}

const TodoApi = new TodoApiClient(Config.todoApiUrl!)
export default TodoApi

