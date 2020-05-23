import React from "react";
import {createStyles, makeStyles, Theme, useTheme} from "@material-ui/core/styles";
import Grid from "@material-ui/core/Grid";
import Typography from "@material-ui/core/Typography";
import Button from "@material-ui/core/Button";
import TextField from "@material-ui/core/TextField";

const useStyles = makeStyles((theme: Theme) =>
  createStyles({}),
);


const AddTaskForm = () => {
  const clases = useStyles()

  return (
    <Grid container direction='column'>
      <Grid item>
        <Typography variant='h4' style={{lineHeight: 2}}>Add Task</Typography>
      </Grid>
      <Grid item>
        <TextField
          label='Title' id='title'
          style={{width: '50%'}}
        />
      </Grid>
      <Grid item>
        <TextField
          label='Content' id='content'
          multiline
          rows={10}
          fullWidth
          margin='normal'
        />
      </Grid>
    </Grid>
)
}

export default AddTaskForm;
