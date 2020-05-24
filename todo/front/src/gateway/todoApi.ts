import Config from "../Config";

class TodoApiClient {
 constructor(private readonly url: string) {
   this.url = url;
 }

 // fetch()のresponseを使う
 async GET(path: string): Promise<any> {
    return fetch(`${this.url}${path}`)
 }
}

const TodoApi = new TodoApiClient(Config.todoApiUrl!)
export default TodoApi

