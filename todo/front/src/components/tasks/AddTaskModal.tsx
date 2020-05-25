import React from 'react';
import {connect} from 'react-redux';
import { makeStyles, Theme, createStyles } from '@material-ui/core/styles';
import Modal from '@material-ui/core/Modal';
import Backdrop from '@material-ui/core/Backdrop';
import Fade from '@material-ui/core/Fade';
import AddTaskForm from "./AddTaskForm";
import {closeAddTaskModal} from "../../actions/taskAction";

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    modal: {
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
    },
    paper: {
      // backgroundColor: theme.palette.background.paper,
      backgroundColor: '#ffffff',
      border: '2px solid #000',
      boxShadow: theme.shadows[1],
      padding: theme.spacing(2, 4, 3),
      width: '80%',
      height: '80%',
      borderRadius: '4px',
      overflow: 'auto',
    },
  }),
);

export interface AddTaskModalProps {
  taskState: {
    openAddTaskModal: boolean,
  }
  closeAddTaskModal: any,
}

const AddTaskModal = (props:AddTaskModalProps) => {
  const {taskState: {openAddTaskModal}, closeAddTaskModal} = props;
  const classes = useStyles();
  return (
    <div>
      <Modal
        aria-labelledby="transition-modal-title"
        aria-describedby="transition-modal-description"
        className={classes.modal}
        open={openAddTaskModal}
        onClose={closeAddTaskModal}
        closeAfterTransition
        BackdropComponent={Backdrop}
        BackdropProps={{
          timeout: 500,
        }}
      >
        <Fade in={openAddTaskModal}>
          <div className={classes.paper}>
            <AddTaskForm />
          </div>
        </Fade>
      </Modal>
    </div>
  );
}

const mapStateToProps = (state: any) => ({
  taskState: state.task
});

export default connect(
  mapStateToProps,
  {closeAddTaskModal}
)(AddTaskModal);
