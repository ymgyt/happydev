import React from "react";
import {connect} from 'react-redux';
import {createStyles, makeStyles, Theme} from "@material-ui/core/styles";
import Grid from "@material-ui/core/Grid";
import Typography from "@material-ui/core/Typography";
import Button from "@material-ui/core/Button";
import TextField from "@material-ui/core/TextField";
import {addTask,closeAddTaskModal} from 'actions/taskAction';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    input: {
      width: '50%',
    }
  }),
);

interface AddTaskFormProps {
  addTask: any,
  closeAddTaskModal:any
}

const AddTaskForm = (props: AddTaskFormProps) => {
  const {addTask, closeAddTaskModal} = props;

  const classes = useStyles()

  const [title, setTitle] = React.useState<string | undefined>("initialDummy");
  const [category, setCategory] = React.useState<string | undefined>("initialDummy")
  const [content, setContent] = React.useState<string | undefined>("initialDummy");

  const handle = () => {
    addTask({
      title: title,
      category: category,
      content: content,
    });
    closeAddTaskModal();
  }

  return (
    <Grid container direction='column' spacing={3}>
      <Grid item xs>
        <Typography variant='h5' style={{lineHeight: 2}}>Enter Task</Typography>
      </Grid>
      <Grid item xs>
        <TextField
          label='Title' id='title'
          className={classes.input}
          value={title}
          onChange={(event) => setTitle(event.target.value)}
        />
      </Grid>
      <Grid item xs>
        <TextField
          label='Category' id='category'
          className={classes.input}
          margin='normal'
          value={category}
          onChange={(event) => setCategory(event.target.value)}
        />
      </Grid>
      <Grid item xs>
        <TextField
          label='Content' id='content'
          multiline
          rows={10}
          fullWidth
          margin='normal'
          value={content}
          onChange={(event) => setContent(event.target.value)}
        />
      </Grid>
      <Grid item container justify='flex-end' xs>
        <Grid item xs={2}>
          <Button
            color='primary'
            variant='outlined'
            fullWidth
            onClick={handle}
          >
            Save
          </Button>
        </Grid>
      </Grid>
    </Grid>
  )
}

export default connect(
  null,
  {addTask,closeAddTaskModal},
)(AddTaskForm);
