import Config from '../Config';

export interface fetchTasksParam extends baseFetchParam {
  query: string;
}

export interface baseFetchParam {
  order: {
    key: string;
    asc: boolean;
  };
}

// どこに定義するか迷う
export type Task = {
  id: string;
  title: string;
  category: string;
  content: string;
  created_at: Date;
  updated_at: Date;
};

class TodoApiClient {
  constructor(private readonly url: string) {
    this.url = url;
  }

  async getTasks(given: fetchTasksParam): Promise<{ tasks: Array<Task> }> {
    const param = {
      ...{
        query: '',
        order: { key: 'id', asc: true },
      },
      ...given,
    };
    // interfaceにmethod定義したい
    // Uri classとかを使う
    let q = '';
    if (param.query) {
      q = `query=${param.query}&`;
    }
    q += `order=${param.order.key}&asc=${param.order.asc}`;
    return this.GET(`/tasks?${q}`);
  }

  async createTask(param: any): Promise<any> {
    return this.POST('/tasks', param);
  }

  async deleteTask(taskId: string): Promise<any> {
    return this.DELETE(`/tasks/${taskId}`);
  }

  private async GET(path: string): Promise<any> {
    return fetch(this.endpoint(path)).then((res) => res.json());
  }

  private async POST(path: string, body: any): Promise<any> {
    return fetch(this.endpoint(path), {
      method: 'POST',
      body: JSON.stringify(body),
      headers: {
        'Content-Type': 'application/json',
      },
    }).then((res) => res.json());
  }

  private async DELETE(path: string): Promise<any> {
    return fetch(this.endpoint(path), {
      method: 'DELETE',
    });
  }

  private endpoint(path: string): string {
    return `${this.url}${path}`;
  }
}

const TodoApi = new TodoApiClient(Config.todoApiUrl!);
export default TodoApi;
