import React from 'react';

export interface TaskProps  {
    id: number;
    title: string;
    content: string;
}

const Task = (props: TaskProps) => {
    return (
        <div className='Task'>
            {props.title}
        </div>
    )
}

export default Task;
