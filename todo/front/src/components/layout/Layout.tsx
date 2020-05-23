import React from 'react';
import {AppBar, Toolbar, Typography, Paper} from '@material-ui/core';

const Layout = (props:any) => (
    <Paper
        elevation={0}
        variant='elevation'
        style={{padding: 0, margin: 0, backgroundColor: '#fafafa'}}
    >
        <AppBar color='primary' position='static' style={{ height: 64 }}>
            <Toolbar style={{ height: 64}}>
                <Typography variant='h6' color='inherit'>TODO</Typography>
            </Toolbar>
        </AppBar>
        {props.children}
    </Paper>
);

export default Layout;
