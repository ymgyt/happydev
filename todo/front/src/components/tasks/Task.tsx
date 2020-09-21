// eslint-disable-next-line no-use-before-define
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
  onDeleteButtonClick?: any,
  onCheckBoxToggle?: any,
}

const Task: React.FC<any> = (props: TaskProps) => {
  const handleDelete = () => {
    props.onDeleteButtonClick(props.id);
  };
  return (
    <ListItem divider={props.divider}>
      <Checkbox
        onClick={props.onCheckBoxToggle}
        checked={props.checked}
        disableRipple={false}
      />
      <ListItemText primary={props.title} />
      <ListItemSecondaryAction>
        <IconButton aria-label="Delete Todo" onClick={handleDelete}>
          <DeleteOutlined />
        </IconButton>
      </ListItemSecondaryAction>
    </ListItem>
  );
};

export default Task;
