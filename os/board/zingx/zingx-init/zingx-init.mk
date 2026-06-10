################################################################################
#
# zingx-init
#
################################################################################

ZINGX_INIT_SITE = $(BR2_EXTERNAL_ZINGX_PATH)/board/zingx/rootfs_overlay
ZINGX_INIT_SITE_METHOD = local
ZINGX_INIT_LICENSE = MIT

define ZINGX_INIT_INSTALL_TARGET_CMDS
	$(INSTALL) -D -m 0755 $(BR2_EXTERNAL_ZINGX_PATH)/board/zingx/rootfs_overlay/sbin/init \
		$(TARGET_DIR)/sbin/init
	$(INSTALL) -D -m 0755 $(BR2_EXTERNAL_ZINGX_PATH)/board/zingx/rootfs_overlay/etc/init.d/S99zingx \
		$(TARGET_DIR)/etc/init.d/S99zingx
	$(INSTALL) -D -m 0644 $(BR2_EXTERNAL_ZINGX_PATH)/board/zingx/rootfs_overlay/etc/inittab \
		$(TARGET_DIR)/etc/inittab
	$(INSTALL) -D -m 0644 $(BR2_EXTERNAL_ZINGX_PATH)/board/zingx/rootfs_overlay/etc/profile \
		$(TARGET_DIR)/etc/profile
	$(INSTALL) -D -m 0755 $(BR2_EXTERNAL_ZINGX_PATH)/board/zingx/rootfs_overlay/usr/bin/zingx-ask \
		$(TARGET_DIR)/usr/bin/zingx-ask
	mkdir -p $(TARGET_DIR)/mnt/data $(TARGET_DIR)/mnt/vault $(TARGET_DIR)/zingx
endef

$(eval $(generic-package))
