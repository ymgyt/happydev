import React from 'react';
import Task, {TaskProps} from './Task';
import AddTask from "./AddTask";
import AddTaskModal from "./AddTaskModal";
import {List, Paper} from '@material-ui/core';

export interface TasksProps {
  tasks: TaskProps[]
  loading: boolean,
}

const Tasks: React.FC<TasksProps> = ({tasks, loading}) => {
  const [openModal, setOpenModal] = React.useState<boolean>(true);

  const handleAddTaskClick = (event: any) => {
    console.log('Add task requested!');
    setOpenModal(true);
  }

  const handleModalClosed = () => {
    console.log('Close Modal');
    setOpenModal(false);
  }
  if (loading) {
    return <h4>Loading...</h4>
  }
  return (
    <Paper style={{margin: 16}}>
      {!loading && tasks.length === 0 ? (<p>no tasks</p>) : (
        <List style={{overflow: 'scroll', padding: 0}}>
          {tasks.map(task =>
            <Task
              key={task.id}
              {...task}
              divider={true}
            />
          )}
        </List>
      )}
      <AddTask onButtonClick={handleAddTaskClick}/>
      <AddTaskModal open={openModal} onClose={handleModalClosed}/>
    </Paper>
  )
}

export default Tasks;
