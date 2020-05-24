import React, {useEffect} from 'react';
import {connect} from 'react-redux';
import Task, {TaskProps} from './Task';
import AddTask from "./AddTask";
import AddTaskModal from "./AddTaskModal";
import {List, Paper} from '@material-ui/core';
import {fetchTasks} from '../../actions/taskAction';

export interface TasksProps {
  taskState: {
    tasks: TaskProps[]
    loading: boolean,
  }
  fetchTasks: any,
}

const Tasks: any = (props:TasksProps) => {
  const {taskState: {tasks, loading}, fetchTasks } = props;

  const [openModal, setOpenModal] = React.useState<boolean>(false);

  useEffect(() => {
    fetchTasks();
    // eslint-disable-next-line
  }, []);

  const handleAddTaskClick = (event: any) => {
    console.log('Add task requested!');
    setOpenModal(true);
  }

  const handleModalClosed = () => {
    console.log('Close Modal');
    setOpenModal(false);
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

const mapStateToProps = (state: any) => ({
  taskState: state.task
});

export default connect(
  mapStateToProps,
  {fetchTasks}
)(Tasks);
