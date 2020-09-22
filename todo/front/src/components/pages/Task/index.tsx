import React from 'react';
import { useHistory, useParams } from 'react-router-dom';
import { Task as TaskModel } from 'model';
import { Container, Grid, Button } from '@material-ui/core';

export type Props = {
  task: TaskModel;
};

const Task: React.FC<Props> = ({ task }) => {
  const history = useHistory();
  const { taskId } = useParams();

  // taskId使ってReduxからTaskを取得する..?


  return (
    <Container maxWidth="lg">
      <Grid>
        <Button onClick={() => history.goBack()}>Back</Button>
      </Grid>
      <Grid>
        <h1>{task.title}</h1>
      </Grid>
      <Grid>
        <h1>{task.content}</h1>
      </Grid>
    </Container>
  );
};

export default Task;
