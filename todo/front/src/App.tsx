import React, {Fragment, useState} from 'react';
import Tasks from './components/tasks/Tasks';
import {Button, Container} from '@material-ui/core';
import './App.css';
import CssBaseline from '@material-ui/core/CssBaseline';


function App() {
    const [name, setName] = useState<string | null>('ymgyt');

    const tasks = [
        {id: 1, title: 'task 1', content: "aaa"},
        {id: 2, title: 'task 2', content: "aaa"},
        {id: 3, title: 'task 3', content: "aaa"},
        {id: 4, title: 'task 4', content: "aaa"},
    ];

    return (
        <Fragment>
            <CssBaseline/>
            <Container className='Container'>
                <Tasks tasks={tasks}/>
                <Button variant="contained" color="primary">
                    Push Me
                </Button>
            </Container>
        </Fragment>
    );
}

export default App;
