(() => {
  const closeMenus = (exceptId = "") => {
    document.querySelectorAll(".inbox-action-menu").forEach((menu) => {
      if (menu.id !== exceptId) menu.classList.add("hidden");
    });
  };

  const syncSelection = () => {
    const selectAll = document.getElementById("selectAllMessages");
    const selected = [...document.querySelectorAll(".message-checkbox:checked")];
    const all = [...document.querySelectorAll(".message-checkbox")];
    const toolbar = document.getElementById("bulkMessageActions");
    const count = document.getElementById("selectedMessageCount");
    const form = document.getElementById("bulkDeleteMessagesForm");
    const submitButton = document.getElementById("bulkDeleteMessagesButton");

    if (selectAll) {
      selectAll.checked = all.length > 0 && selected.length === all.length;
      selectAll.indeterminate = selected.length > 0 && selected.length < all.length;
    }
    if (toolbar) {
      toolbar.classList.toggle("hidden", selected.length === 0);
      toolbar.classList.toggle("flex", selected.length > 0);
    }
    if (count) count.textContent = selected.length;
    if (submitButton) submitButton.disabled = selected.length === 0;
    if (form) {
      const noun = selected.length === 1 ? "enquiry" : "enquiries";
      form.dataset.confirm = `Delete ${selected.length} selected ${noun} permanently? This cannot be undone.`;
    }
  };

  const init = () => {
    document.querySelectorAll("[data-contact-filters-toggle]").forEach((button) => {
      button.addEventListener("click", () => {
        document.getElementById("contactFilters")?.classList.toggle("hidden");
      });
    });
    document.querySelectorAll("[data-contact-filters-close]").forEach((button) => {
      button.addEventListener("click", () => document.getElementById("contactFilters")?.classList.add("hidden"));
    });
    document.querySelectorAll("[data-inbox-menu-toggle]").forEach((button) => {
      button.addEventListener("click", (event) => {
        event.preventDefault();
        event.stopPropagation();
        const menuId = button.dataset.inboxMenuToggle;
        const menu = menuId ? document.getElementById(menuId) : null;
        if (!menu) return;
        const opening = menu.classList.contains("hidden");
        closeMenus(menuId);
        menu.classList.toggle("hidden", !opening);
      });
    });
    document.addEventListener("click", (event) => {
      if (!event.target.closest(".inbox-action-menu")) closeMenus();
    });
    document.addEventListener("keydown", (event) => {
      if (event.key === "Escape") closeMenus();
    });
    const selectAll = document.getElementById("selectAllMessages");
    selectAll?.addEventListener("change", () => {
      document.querySelectorAll(".message-checkbox").forEach((checkbox) => {
        checkbox.checked = selectAll.checked;
      });
      syncSelection();
    });
    document.querySelectorAll(".message-checkbox").forEach((checkbox) => {
      checkbox.addEventListener("change", syncSelection);
    });
    document.querySelectorAll("form[data-confirm]").forEach((form) => {
      form.addEventListener("submit", (event) => {
        if (!window.confirm(form.dataset.confirm)) event.preventDefault();
      });
    });
    syncSelection();
  };

  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", init);
  else init();
})();
