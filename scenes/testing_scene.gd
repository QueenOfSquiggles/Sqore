extends Node

func _ready() -> void:
	CoreDialog.load_track("res://Dialogic/example.json")
	get_tree().quit()
