# curiosity-os
On-board software for Curiosity, a playful spaceship simulator

A high-level design for Curiosity can be found at https://www.adtme.com/projects/Curiosity

CuriosityOS consists of two main components:
- A user interface for managing the spaceship
- A simulator that keeps track of the spaceship and the universe around it

## User Interface
The user interface includes the following:
- A shell where the user can run commands
- Some simple readouts about spaceship status
- Basic utilities for planning your adventures and communicating with mission control

The user interface is intentionally a little opaque. This means that you have to explore a little in order to find all its features.

## Simulator
The simulator maintains a model that represents the state of the spaceship and the universe. The model is updated in a simulation loop based on a set of inputs. Those inputs include hardware (switches, buttons), the current state, and settings from the UI. At the end of each loop, the newly simulated state is used to update a set of hardware outputs such as lights or speakers.

Information about the model can be queried (for example via the UI). The model is periodically saved to disk to ensure that if the system crashes, it can come back up without losing too much information.
