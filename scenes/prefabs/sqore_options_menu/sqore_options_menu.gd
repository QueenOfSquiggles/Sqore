extends VBoxContainer
"""
Relatively system agnostic menu for use of quickly bootstrapping an options menu for your game. If you want to create a custom menu this may serve as a template and example of how to interface with Sqore's various system settings
"""

## When entering the tree, forces Sqore global settings to be loaded from disk (in cast that hasn't happened yet)
func _enter_tree() -> void:
	Sqore.reload_globals()

## When exiting the tree, force Sqore to save to disk any changes that have been made. This helps to prevent any dissonance between runtime globals and disk.
func _exit_tree() -> void:
	Sqore.save_globals()
