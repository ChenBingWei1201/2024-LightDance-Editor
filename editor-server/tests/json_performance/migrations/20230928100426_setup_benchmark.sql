-- Add migration script here
CREATE TABLE `LEDEffect` (
  `id` INTEGER NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(191) NOT NULL,
  `part_name` VARCHAR(191) NOT NULL,
  `repeat` INTEGER NOT NULL,
  `frames` JSON NOT NULL,

  UNIQUE INDEX `led_effect_name_part_name_key`(`name`, `part_name`),
  PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

CREATE TABLE `LEDEffectTest` (
    `id` INTEGER NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(191) NOT NULL,
    `part_name` VARCHAR(191) NOT NULL,
    `repeat` INTEGER NOT NULL,

    UNIQUE INDEX `led_effect_test_name_part_name_key`(`name`, `part_name`),
    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

CREATE TABLE `LED` (
    `effect_id` INTEGER NOT NULL,
    `position` INTEGER NOT NULL,
    `color_id` INTEGER NOT NULL,
    `alpha` INTEGER NOT NULL,

    UNIQUE INDEX `led_effect_id_position_key`(`effect_id`, `position`),
    PRIMARY KEY (`effect_id`, `position`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

ALTER TABLE `LED` ADD CONSTRAINT `led_effect_id_fkey` FOREIGN KEY (`effect_id`) REFERENCES `LEDEffectTest`(`id`) ON DELETE CASCADE ON UPDATE CASCADE;
