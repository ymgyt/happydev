import React from 'react';
import {Paper, Button, Grid} from '@material-ui/core';

const AddTask = (props: any) => {
  return (
    <Paper style={{margin: 0, padding: 10}}>
      <Grid container justify='flex-end'>
        <Grid xs={2} item>
          <Button
            fullWidth
            color='primary'
            variant='outlined'
            onClick={props.onButtonClick}
          >
            Add
          </Button>
        </Grid>
      </Grid>
    </Paper>
  )
};

export default AddTask;
