import React from 'react';
import {TextField, Paper, Button, Grid} from '@material-ui/core';

const AddTodo = (props: any) => (
  <Paper style={{margin: 0, padding: 10}}>
    <Grid container>
      <Grid md={11} item style={{paddingRight: 16}}>
        <TextField
          placeholder="Add Todo here"
          value={props.inputValue}
          onChange={props.onInputChange}
          onKeyPress={props.onInputKeyPress}
          fullWidth
        />
      </Grid>
      <Grid md={1} item>
        <Button
          fullWidth
          color='secondary'
          variant='outlined'
          onClick={props.onButtonClick}
        >
          Add
        </Button>
      </Grid>
    </Grid>
  </Paper>
);

export default AddTodo;