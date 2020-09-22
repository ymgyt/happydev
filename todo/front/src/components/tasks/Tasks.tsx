// eslint-disable-next-line no-use-before-define
import React, { useEffect } from 'react';
import { connect } from 'react-redux';
import { List, Paper } from '@material-ui/core';
import { fetchTasks, openAddTaskModal, deleteTask } from 'actions/taskAction';
import Task, { TaskProps } from './Task';
import AddTask from './AddTask';
import AddTaskModal from './AddTaskModal';

export interface TasksProps {
  taskState: {
    tasks: TaskProps[];
    loading: boolean;
  };
  fetchTasks: any;
  openAddTaskModal: any;
  deleteTask: any;
}

const Tasks: React.FC<TasksProps> = ({
  taskState: { tasks, loading },
  fetchTasks,
  openAddTaskModal,
  deleteTask,
}) => {
  useEffect(() => {
    fetchTasks({ query: '', order: { key: 'created_at', asc: true } });
    // eslint-disable-next-line
  }, []);

  const handleAddTaskClick = (event: any) => {
    openAddTaskModal();
  };

  if (loading || tasks === null) {
    return <h4>Loading...</h4>;
  }
  return (
    <Paper style={{ margin: 16 }}>
      {!loading && tasks.length === 0 ? (
        <p>no tasks</p>
      ) : (
        <List style={{ overflow: 'scroll', padding: 0 }}>
          {tasks.map((task) => (
            <Task
              key={task.id}
              {...task}
              onDeleteButtonClick={deleteTask}
              divider
            />
          ))}
        </List>
      )}
      <AddTask onButtonClick={handleAddTaskClick} />
      <AddTaskModal />
    </Paper>
  );
};

const mapStateToProps = (state: any) => ({
  taskState: state.task,
});

export default connect(mapStateToProps, {
  fetchTasks,
  openAddTaskModal,
  deleteTask,
})(Tasks);
