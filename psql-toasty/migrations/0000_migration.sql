CREATE TABLE "subject_roles" (
    "id" UUID NOT NULL,
    "subject_id" UUID NOT NULL,
    "role_id" UUID NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE INDEX "index_subject_roles_by_subject_id" ON "subject_roles" ("subject_id");
CREATE UNIQUE INDEX "index_subject_roles_by_role_id" ON "subject_roles" ("role_id");
CREATE TABLE "roles" (
    "id" UUID NOT NULL,
    "identifier" TEXT NOT NULL,
    "description" TEXT,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_roles_by_identifier" ON "roles" ("identifier");
CREATE TABLE "permissions" (
    "id" UUID NOT NULL,
    "resource" TEXT NOT NULL,
    "action" TEXT NOT NULL,
    "description" TEXT,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_permissions_by_action" ON "permissions" ("action");
CREATE TABLE "server_roles" (
    "id" UUID NOT NULL,
    "description" TEXT,
    "role_id" UUID NOT NULL,
    "server_id" UUID NOT NULL,
    "subject_id" UUID NOT NULL,
    "granted_by" UUID NOT NULL,
    PRIMARY KEY ("id")
);
CREATE INDEX "index_server_roles_by_role_id" ON "server_roles" ("role_id");
CREATE INDEX "index_server_roles_by_server_id" ON "server_roles" ("server_id");
CREATE INDEX "index_server_roles_by_subject_id" ON "server_roles" ("subject_id");
CREATE INDEX "index_server_roles_by_granted_by" ON "server_roles" ("granted_by");
CREATE TABLE "servers" (
    "id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "description" TEXT,
    "address" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_servers_by_name" ON "servers" ("name");
CREATE TABLE "users" (
    "id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE TABLE "role_permissions" (
    "id" UUID NOT NULL,
    "role_id" UUID NOT NULL,
    "permission_id" UUID NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE INDEX "index_role_permissions_by_role_id" ON "role_permissions" ("role_id");
CREATE UNIQUE INDEX "index_role_permissions_by_permission_id" ON "role_permissions" ("permission_id");
