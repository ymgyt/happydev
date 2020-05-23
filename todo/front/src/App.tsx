import React, {useState,useEffect} from 'react';
import Tasks, {TasksProps} from './components/tasks/Tasks';
import Layout from "./components/layout/Layout";
import './App.css';
import CssBaseline from '@material-ui/core/CssBaseline';


const App = () => {

  useEffect(() => {
    console.log("initialize app...");
  })

  const initialTasks: TasksProps = {
    tasks: [
      {id: 1, title: 'task 1', content: "aaa"},
      {id: 2, title: 'task 2', content: "aaa"},
      {id: 3, title: 'task 3', content: "aaa"},
      {id: 4, title: 'task 4', content: "aaa"},
    ]
  };

  const [tasks, setTask] = useState(initialTasks);

  return (
      <Layout>
        <CssBaseline/>
        <Tasks {...tasks}/>
      </Layout>
  );
}

export default App;
