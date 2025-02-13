# General
app-name = Flighty
app-description = Flutter application deployment and management CLI tool

# Commands
cmd-init = Initialize a new project
cmd-init-path = Project directory

cmd-config = View or edit configuration
cmd-config-key = Configuration key
cmd-config-value = New value

cmd-build = Build the application
cmd-build-platform = Build platform (android/ios)
cmd-build-version = Version number
cmd-build-number = Build number
cmd-build-channel = Release channel (alpha/beta/production)

cmd-deploy = Deploy the application
cmd-deploy-platform = Deployment platform (android/ios)
cmd-deploy-target = Deployment target (store/firebase)

cmd-flutter = Manage Flutter SDK
cmd-flutter-command = Flutter command

cmd-language = Change language
cmd-language-code = Language code (en/tr)

# Messages
msg-project-init = Initializing project: {$path}
msg-config-exists = Configuration file already exists
msg-config-created = Default configuration created
msg-flutter-missing = Flutter project not found. Please run 'flutter create' first
msg-build-start = Starting build: {$platform}
msg-build-complete = Build completed: {$platform}
msg-deploy-start = Starting deployment: {$platform} -> {$target}
msg-deploy-complete = Deployment completed: {$platform}
msg-invalid-platform = Invalid platform
msg-invalid-target = Invalid target
msg-invalid-channel = Invalid channel
msg-flutter-not-found = Flutter SDK not found
msg-language-changed = Language changed to: {$lang}

# Errors
error-config = Configuration error: {$message}
error-flutter = Flutter CLI error: {$message}
error-io = IO error: {$message}
error-invalid-arg = Invalid argument: {$message}
error-unknown = Unknown error: {$message} 