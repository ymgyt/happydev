import React from 'react';
import {
  ListItem,
  Checkbox,
  IconButton,
  ListItemText,
  ListItemSecondaryAction,
} from '@material-ui/core';
import DeleteOutlined from '@material-ui/icons/DeleteOutlined';

export interface TaskProps {
  // data
  id: string,
  title: string,
  category: string,
  content: string,

  // ui
  divider?: boolean,
  checked?: boolean,
  onButtonClick?: any,
  onCheckBoxToggle?: any,
}

const Task: React.FC<any> = (props: TaskProps) => {
  return (
    <ListItem divider={props.divider}>
      <Checkbox
        onClick={props.onCheckBoxToggle}
        checked={props.checked}
        disableRipple={false}
      />
      <ListItemText primary={props.title}/>
      <ListItemSecondaryAction>
        <IconButton aria-label='Delete Todo' onClick={props.onButtonClick}>
          <DeleteOutlined/>
        </IconButton>
      </ListItemSecondaryAction>
    </ListItem>
  )
}

export default Task;
