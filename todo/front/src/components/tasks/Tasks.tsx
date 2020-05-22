import React from 'react';
import Task, {TaskProps} from './Task';

export interface TasksProps {
    tasks: TaskProps[]
}

const Tasks = (props: TasksProps) => {
    return (
        <div className='Tasks'>
            {props.tasks.map(task =>
                <Task key={task.id} id={task.id} title={task.title} content={task.content}/>
            )}
        </div>
    )
}

export default Tasks;
