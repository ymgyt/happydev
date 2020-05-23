import React from 'react';
import Task, {TaskProps} from './Task';
import AddTodo from "./AddTask";
import {List, Paper} from '@material-ui/core';

export interface TasksProps {
  tasks: TaskProps[]
  loading: boolean,
}

const Tasks: React.FC<TasksProps> = ({tasks, loading}) => {
  const handleClick = (event: any) => {
    console.log('Add task requested!');
  }
  if (loading) {
    return <h4>Loading...</h4>
  }
  return (
    <Paper style={{margin: 16}}>
      <List style={{overflow: 'scroll', padding: 0}}>
        {tasks.map(task =>
          <Task
            key={task.id}
            {...task}
            divider={true}
          />
        )}
        <AddTodo/>
      </List>
    </Paper>
  )
}

export default Tasks;
