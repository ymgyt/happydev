import React from "react";
import {createStyles, makeStyles, Theme} from "@material-ui/core/styles";
import Grid from "@material-ui/core/Grid";
import Typography from "@material-ui/core/Typography";
import Button from "@material-ui/core/Button";
import TextField from "@material-ui/core/TextField";

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    input: {
      width: '50%',
    }
  }),
);


const AddTaskForm = () => {
  const classes = useStyles()

  const [title, setTitle] = React.useState<string|undefined>("");
  const [category, setCategory] = React.useState<string|undefined>("")
  const [content, setContent] = React.useState<string|undefined>("");

  const handle = () => {
    console.log("save clicked", title, content);
  }

  return (
    <Grid container direction='column'>
      <Grid item>
        <Typography variant='h5' style={{lineHeight: 2}}>Enter Task</Typography>
      </Grid>
      <Grid item>
        <TextField
          label='Title' id='title'
          className={classes.input}
          value={title}
          onChange={(event) => setTitle(event.target.value)}
        />
      </Grid>
      <Grid item>
        <TextField
          label='Category' id='category'
          className={classes.input}
          margin='normal'
          value={category}
          onChange={(event)=> setCategory(event.target.value)}
        />
      </Grid>
      <Grid item>
        <TextField
          label='Content' id='content'
          multiline
          rows={10}
          fullWidth
          margin='normal'
          value={content}
          onChange={(event)=> setContent(event.target.value)}
        />
      </Grid>
     <Grid item container justify='flex-end'>
       <Grid item md={2}>
         <Button
           style={{marginTop: '10px'}}
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

export default AddTaskForm;
