BASELINE ?= false

benchmark = cargo bench \
	--bench $(NAME) \
	-- --verbose \
	$(if $(BASELINE),--save-baseline main,--baseline main)

.PHONY: benchmark-component
benchmark-component:
	@$(call log,"running $(NAME) benchmarks")
	@$(benchmark)

benchmark-calculator: NAME=calculator
benchmark-calculator: benchmark-component ## quality: benchmark calculator

CPU_PROFILE_BUILD_CMD ?= cargo build --release
CPU_PROFILE_CMD ?= samply record --save-only
CPU_PROFILE_LOAD_CMD ?= samply load
CPU_PROFILE_PATH ?= target/profiles

.PHONY: cpu-profile-component
cpu-profile-component:
	@$(call log,"building $(NAME) profiling benchmark")
	@$(CPU_PROFILE_BUILD_CMD) \
		--example profile_$(NAME)
	@$(call log,"profiling $(NAME)")
	@mkdir -p $(CPU_PROFILE_PATH)
	@$(CPU_PROFILE_CMD) \
		-o $(CPU_PROFILE_PATH)/$(NAME).json \
		cargo run \
		--release \
		--example profile_$(NAME) \
		-- baseline
	@$(CPU_PROFILE_LOAD_CMD) $(CPU_PROFILE_PATH)/$(NAME).json

.PHONY: cpu-profile-calculator
cpu-profile-calculator: NAME=calculator
cpu-profile-calculator: cpu-profile-component ## quality: CPU profile calculator
