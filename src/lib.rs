#![deny(missing_docs)]

/*!
# Compensation Tracker

This crate implements Compensation Tracker(\<insert-paper-here\>) as well as
the so called 'Basic Tracker'.

The coupling is done for 2 reasons:
  - Ease of Implementation
  - Possibly performance gain

Anyway, this crate relies on following concepts:
  - Detection - Something received from e.g. Neural Network and has no
    information about itself aside from bounding box, confidence and classification
  - Tracklet - The main idea behind this module. For now on surface level it is
    simply a series of detections which keeps track of it's movements and classification
  - Basic Tracker performing 'Motion Compensation' with a Kallman Filter and top
    level assignment of detections to tracklets
  - Compensation Tracker - A tracking approach designed to handle detector's
    shortcomings and mutual tracklet occlusion

*/



