import Config from "../../Config";

class TodoApiClient {
 constructor(private readonly url: string) {
   this.url = url;
 }

 // fetch()のresponseを使う
 async GET(path: string): Promise<any> {
   if (path === '/tasks') {
     // mock response
     return new Promise((resolve, reject) => {
       resolve({
         tasks: [
           {id: 1, title: 'task 1', content: "aaa"},
           {id: 2, title: 'task 2', content: "aaa"},
           {id: 3, title: 'task 3', content: "aaa"},
           {id: 4, title: 'task 4', content: "aaa"},
         ]
       })
     })
   }
    return fetch(`${this.url}${path}`)
 }
}

const TodoApi = new TodoApiClient(Config.todoApiUrl!)
export default TodoApi

