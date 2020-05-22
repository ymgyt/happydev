import React from 'react';

type TasksProps = {
    message: string;
}

const Tasks = (props: TasksProps) => {
  return (
    <ol>
      <li>Task 1 {props.message}</li>
      <li>Task 2</li>
    </ol>
  )
}

export default Tasks;
