extends Node

func _ready() -> void:
	CoreDialog.blackboard_action("set a 0")
	assert(CoreDialog.blackboard_query("a == 0"))
	CoreDialog.blackboard_action("add a 1")
	assert(CoreDialog.blackboard_query("a == 1"))
	CoreDialog.blackboard_action("sub a 2")
	assert(CoreDialog.blackboard_query("a == -1"))


	CoreDialog.blackboard_debug_dump()
	#CoreDialog.load_track("res://Dialogic/example.json")
	get_tree().quit()
