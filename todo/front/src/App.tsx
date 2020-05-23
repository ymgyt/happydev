import React, {useState, useEffect} from 'react';
import Tasks, {TasksProps} from './components/tasks/Tasks';
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
    <div className={classes.app}>
      <Layout>
        <CssBaseline/>
        <Tasks {...tasks}/>
      </Layout>
    </div>
  );
}

export default App;
