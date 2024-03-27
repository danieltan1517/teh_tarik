#!/bin/bash
# Set up any custom environment variables that will be used in the container. 
export APPTAINERENV_PS1='[cs152 container \u@\h \W]\$ '
export APPTAINER_BIND=/usr/local

# All the software for the course is available in the container, so start a 
# shell in it to do work.
singularity shell /usr/local/containers/cs152.sif
