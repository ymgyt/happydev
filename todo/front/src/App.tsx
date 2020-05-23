import React, {useState, useEffect} from 'react';
import Tasks, {TasksProps} from './components/tasks/Tasks';
import {TaskProps} from './components/tasks/Task';
import Config from "./Config";
import TodoApi from "./components/gateway/todoApi";
import Layout from "./components/layout/Layout";
import CssBaseline from '@material-ui/core/CssBaseline';
import {makeStyles} from '@material-ui/styles';

const useStyles = makeStyles((theme) => ({
  app: {
    fontFamily: 'Roboto',
  },
}));


const App = () => {
  const classes = useStyles();

  const [tasks, setTasks] = useState<TaskProps[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect( () => {
    console.log("initialize app...", Config);
    getTasks();
  },[])


  const getTasks = async () => {
    setLoading(true);
    const res = await TodoApi.GET('/tasks');
    console.log("got tasks", res);
    setTasks(res.tasks);
    setLoading(false);
  }

  return (
    <div className={classes.app}>
      <Layout>
        <CssBaseline/>
        <Tasks tasks={tasks} loading={loading}/>
      </Layout>
    </div>
  );
}

export default App;
