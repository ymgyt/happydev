import React, {useEffect} from 'react';
import {connect} from 'react-redux';
import Task, {TaskProps} from './Task';
import AddTask from "./AddTask";
import AddTaskModal from "./AddTaskModal";
import {List, Paper} from '@material-ui/core';
import {fetchTasks, openAddTaskModal, deleteTask} from '../../actions/taskAction';

export interface TasksProps {
  taskState: {
    tasks: TaskProps[]
    loading: boolean,
  }
  fetchTasks: any,
  openAddTaskModal: any,
  deleteTask: any,
}

const Tasks: any = (props: TasksProps) => {
  const {taskState: {tasks, loading}, fetchTasks, openAddTaskModal, deleteTask} = props;

  useEffect(() => {
    fetchTasks();
    // eslint-disable-next-line
  }, []);

  const handleAddTaskClick = (event: any) => {
    openAddTaskModal()
  }

  if (loading || tasks === null) {
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
              onDeleteButtonClick={deleteTask}
              divider={true}
            />
          )}
        </List>
      )}
      <AddTask onButtonClick={handleAddTaskClick}/>
      <AddTaskModal/>
    </Paper>
  )
}

const mapStateToProps = (state: any) => ({
  taskState: state.task
});

export default connect(
  mapStateToProps,
  {fetchTasks, openAddTaskModal,deleteTask}
)(Tasks);
